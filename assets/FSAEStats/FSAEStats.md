Motor racing, as a sport, is statistics-adjacent. In the
same way that a baseball game, or an entire series, can be recreated from
the key measures of players, the *story* of a race can be told almost
entierly from the statistics. The ups and downs and emotional highs of
a race, of course, can't quite be quantified (yet), but the image of
a driver, of their personality and strategy and legacy, is told in half
floats entries on a CSV. 

Despite this, of course, the Formula Student competition provides the
results of the yearly competition in a PDF format, which I would say is
almost hostile to any form of analysis. And if it's hostile to
excel-spreadsheet-accountant type data analysis, it is outright aggressive
towards any attempts to do any data *science* with it. 

Extracting the data from the PDF, which is in tabular format already, is
actually quite easy with ```Camelot```. I honestly cannot reccomend this
package enough; I spent [far too
much](https://github.com/NicoOhR/FSAERawResults/commits/main/) time trying
to clean up last years data which I extracted with ```PdfPlumber```. Once
we have a CSV we can do as we'd like with the data, I wrote up a small
analysis on last years competition that I would like to revise, and will
probably push out on here soon (tm). 

What follows is a largely unnecessary exercise in data engineering;
there's not enough data to necessitate a whole system to distribute it. It
would be just as easy to send someone CSVs, but what the hell, rust is
fun. The goal from the outset was to have an API that would take care of
the more tedious work of inner-joins and key-searches, and provide the end
user with an effecient format to work with. The final state of this
project is something that I am pretty happy with, even if getting there
was very messy. I wanted to keep the codebase lean, and reduce the amount
of boilerplate necessary, this precluded for me most ORMs. Since the most
natural representation of the data is as each event (autocross,
acceleration, endurance, etc.) as its own table in the database, since
each event has it's own fields and measures, that means that an ORM would
create a different struct for every single event, and then, in the rust
code, force an inordinante amount of very similar looking ```match```
statements. Is this *really* an issue? No, not really, it just looks ugly.
Thus, begins the somewhat painful journey from Diesal to sqlX to SeaORM to
whatever else I could find. I frankly don't even remember some of the
experiments I made in forks that were never pushed.

What I wanted was to be able to handle everything generically and quickly.
A column/row from the acceleration results table is not essentially
different from a column/row from the endurance results table. I also
wanted to be flexible, all of which (and this is mostly a skill issue
I think) was pretty difficult with an ORM. So instead I opted to try using
```DuckDB``` and communicate to my client with the Apache Arrow format.
Which, honestly, was only a little painful!

Critically, DuckDB allows the user to treat rows generically, and simply
treat every request as resulting in a ```RecordBatch```.

Now to the code, which is really (and proudly) simple. Practically, we
define a ```UserRequest``` Struct, implement for it parsing and ```handle``` methods, 
and use the information to query DuckDB:

```Rust 
pub struct UserRequest {
    pub team: String,
    pub year: String,
    pub event: String,
}
...
fn from_hash(args: &mut HashMap<String, String>) -> Result<Box<Self>, ParseError> {
    let team = args.remove("team").ok_or(ParseError::Missing("team"))?;
    let year = args.remove("year").ok_or(ParseError::Missing("year"))?;
    let event = args.remove("event").ok_or(ParseError::Missing("event"))?;

    Ok(Box::new(Self { team, year, event }))
}
async fn handle(self, conn: duckdb::Connection) -> Result<Vec<arrow::array::RecordBatch>> {
    let query: String = format!("SELECT * FROM {} WHERE Team = '{}'", self.event, self.team);
    let mut stmt = conn.prepare(&query)?;
    let rbs = stmt.query_arrow([])?.collect();
    Ok(rbs)
}
```

Then, from our server handler, we can easily leverage those functions to
generate the binary package

```Rust
match (req.method(), req.uri().path()) {
    (&Method::GET, "/") => Ok(Response::new(full("GET the
    /team/year/event"))),
    (&Method::GET, "/event_arrow") => { 
    let mut request = parse_request(req).await?;
    let response = UserRequest::from_hash(&mut request)?.handle(pool).await?; 
    let mut buf = Cursor::new(Vec::new());
    { 
        let mut writer = arrow::ipc::writer::StreamWriter::try_new(&mut buf, &*response[0].schema())?;

        response.iter().for_each(|x| writer.write(x).unwrap());

        writer.finish()?;
    } 
    Ok(Response::new(full(buf.into_inner()))) 
} 
```

From here, I anticipate adding a couple of things. Firstly, I need to add
the rest of the years to the database, which I've put off primarily
because of tedium. Then, I'd also like to give the user some more
convinient end points, like, for example the option to sample all years of
a teams performance for a specific event, or even a ```summary``` end
point which would give the user some convinient statistics. Finally, just
to finish it off I'd like to add Swagger Documentation for this API, and
maybe spend a weekend cobbeling together a front end exhibition for this.
