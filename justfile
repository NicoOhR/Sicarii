alias r := run

run:
  cargo run
  npm run build:css

serve:
  python3 -m http.server --directory static > /dev/null 2>&1 &
  chromium --new-window http://localhost:8000 > /dev/null 2>&1 &
  npm run watch:css

gh-deploy:
  git add .
  git commit -m "content update"
  git push
  git subtree push --prefix site origin gh-pages

#sometimes subtree decides local is behind remote
#this force pushes local to remote with subtree
gh-force-push:
  git push origin `git subtree split --prefix site main`:gh-pages --force
