#!/usr/bin/env python3
import re
import unicodedata
import subprocess
from dataclasses import dataclass
from datetime import date
from pathlib import Path
from typing import Optional

ROOT = Path(__file__).resolve().parents[1]
ASSETS_DIR = ROOT / "assets"
OUTPUT_TEX = ROOT / "portfolio.tex"

SPLICE_RE = re.compile(r"^\{\{\{(.+?)\}\}\}\s*$", re.MULTILINE)
SIDENOTE_RE = re.compile(
    r"<label[^>]*></label>\s*<input[^>]*/>\s*<span class=\"sidenote\">([^<]*)</span>"
)


@dataclass
class Entry:
    title: str
    subtitle: Optional[str]
    author: Optional[str]
    date: Optional[date]
    link: Optional[str]
    content_path: Path


def escape_latex(text: str) -> str:
    replacements = {
        "\\": r"\textbackslash{}",
        "&": r"\&",
        "%": r"\%",
        "$": r"\$",
        "#": r"\#",
        "_": r"\_",
        "{": r"\{",
        "}": r"\}",
        "~": r"\textasciitilde{}",
        "^": r"\textasciicircum{}",
    }
    return "".join(replacements.get(ch, ch) for ch in text)


def parse_date(raw: Optional[str]) -> Optional[date]:
    if not raw:
        return None
    try:
        return date.fromisoformat(raw)
    except ValueError:
        return None


def load_entries() -> list[Entry]:
    entries: list[Entry] = []
    for toml_path in sorted(ASSETS_DIR.glob("**/*.toml")):
        data = toml_path.read_bytes()
        try:
            import tomllib

            parsed = tomllib.loads(data.decode("utf-8"))
        except Exception:
            continue
        content_rel = parsed.get("content_path")
        if not content_rel:
            continue
        content_path = ROOT / content_rel
        entries.append(
            Entry(
                title=parsed.get("title", toml_path.stem),
                subtitle=parsed.get("subtitle"),
                author=parsed.get("author"),
                date=parse_date(parsed.get("date")),
                link=parsed.get("link"),
                content_path=content_path,
            )
        )
    entries.sort(key=lambda e: e.date or date.min, reverse=True)
    return entries


def preprocess_markdown(text: str) -> str:
    text = SPLICE_RE.sub(r"*Embedded asset:* \1\n\n", text)
    text = SIDENOTE_RE.sub(r" (Sidenote: \1) ", text)
    text = text.replace("♥", "<3")
    text = unicodedata.normalize("NFKD", text).encode("ascii", "ignore").decode("ascii")
    return text


def markdown_to_latex(text: str) -> str:
    result = subprocess.run(
        ["pandoc", "-f", "markdown", "-t", "latex", "--wrap=preserve"],
        input=text.encode("utf-8"),
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=True,
    )
    return result.stdout.decode("utf-8")


def build_document(entries: list[Entry]) -> str:
    parts: list[str] = []
    parts.append(r"\documentclass[11pt]{article}")
    parts.append(r"\usepackage[margin=1in]{geometry}")
    parts.append(r"\usepackage[T1]{fontenc}")
    parts.append(r"\usepackage{lmodern}")
    parts.append(r"\usepackage{hyperref}")
    parts.append(r"\usepackage{amsmath,amssymb}")
    parts.append(r"\usepackage{graphicx}")
    parts.append(r"\usepackage{longtable}")
    parts.append(r"\usepackage{booktabs}")
    parts.append(r"\usepackage{parskip}")
    parts.append(r"\usepackage{xcolor}")
    parts.append(r"\usepackage{framed}")
    parts.append(r"\usepackage{fancyvrb}")
    parts.append(r"\definecolor{shadecolor}{RGB}{248,248,248}")
    parts.append(r"\newenvironment{Shaded}{\begin{snugshade}}{\end{snugshade}}")
    parts.append(r"\providecommand{\tightlist}{%")
    parts.append(r"  \setlength{\itemsep}{0pt}\setlength{\parskip}{0pt}")
    parts.append(r"}")
    parts.append("")
    parts.append(r"\begin{document}")
    parts.append(r"\title{Portfolio}")
    parts.append(r"\author{}")
    parts.append(r"\date{}")
    parts.append(r"\maketitle")
    parts.append(r"\tableofcontents")
    parts.append(r"\newpage")

    for entry in entries:
        parts.append(r"\section{" + escape_latex(entry.title) + "}")
        if entry.subtitle:
            parts.append(r"\subsection*{" + escape_latex(entry.subtitle) + "}")
        meta_bits = []
        if entry.author:
            meta_bits.append(escape_latex(entry.author))
        if entry.date:
            meta_bits.append(entry.date.isoformat())
        if entry.link:
            meta_bits.append(r"\href{" + entry.link + "}{" + entry.link + "}")
        if meta_bits:
            parts.append(r"\textit{" + " \\textbullet\\  ".join(meta_bits) + "}")
        parts.append("")

        markdown = preprocess_markdown(entry.content_path.read_text(encoding="utf-8"))
        latex_body = markdown_to_latex(markdown).strip()
        parts.append(latex_body)
        parts.append(r"\newpage")

    parts.append(r"\end{document}")
    parts.append("")
    return "\n".join(parts)


def main() -> None:
    entries = load_entries()
    if not entries:
        raise SystemExit("No entries found in assets.")
    document = build_document(entries)
    OUTPUT_TEX.write_text(document, encoding="utf-8")


if __name__ == "__main__":
    main()
