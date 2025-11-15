run:
  cargo run

serve:
  python3 -m http.server --directory site > /dev/null 2>&1 &
  firefox --new-tab --url http://localhost:8000 > /dev/null 2>&1 &

gh-deploy:
  git push origin "$(git subtree split --prefix=site HEAD)":gh-pages -f
