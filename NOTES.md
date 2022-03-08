# Notes

## Get the sizes of blobs

```shell
git rev-list --all --objects | \
  git cat-file --batch-check='%(objecttype) %(objectname) %(objectsize) %(rest)' | \
  sort --human-numeric-sort -k3nr | \
  head | \
  numfmt --field 3 --to=iec
```