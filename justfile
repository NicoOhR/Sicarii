alias r := run

run:
  cargo run

serve:
  python3 -m http.server --directory static > /dev/null 2>&1 &
  chromium --new-window http://localhost:8000

gh-deploy:
  git add .
  git commit -m "content update"
  git push
  git subtree push --prefix static origin gh-pages

#sometimes subtree decides local is behind remote
#this force pushes local to remote with subtree
gh-force-push:
  git push origin `git subtree split --prefix static main`:gh-pages --force
