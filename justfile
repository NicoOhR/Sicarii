alias b := build

build:
  cargo run

serve:
  python3 -m http.server --directory static > /dev/null 2>&1 &
  firefox -new-tab "localhost:8000"

gh-deploy:
  git add .
  git commit -m "content update"
  git push
  git subtree push --prefix static origin gh-pages
