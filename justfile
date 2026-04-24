run:
  cargo run --release

serve:
  python3 -m http.server --directory site > /dev/null 2>&1 &
  google-chrome --new-window http://localhost:8000 > /dev/null 2>&1 &

gh-deploy:
  git push origin "$(git subtree split --prefix=site HEAD)":gh-pages -f
