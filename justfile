run:
  cargo run

serve:
  python3 -m http.server --directory site > /dev/null 2>&1 &
  firefox --new-tab --url http://localhost:8000 > /dev/null 2>&1 &

gh-deploy:
  git add .
  git commit -m "content update"
  git push
  git subtree push --prefix site origin gh-pages

#sometimes subtree decides local is behind remote
#this force pushes local to remote with subtree
gh-force-push:
  git push origin `git subtree split --prefix site main`:gh-pages --force
