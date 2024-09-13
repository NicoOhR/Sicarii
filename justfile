alias b := build

build:
  cargo build

serve:
  python3 -m http.server --directory static > /dev/null 2>&1 &

gh-deploy:
  git subtree push --prefix static origin gh-pages
