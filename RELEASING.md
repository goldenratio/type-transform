# Releasing Type-Transform

```
cargo release <VERSION LEVEL> --execute --no-publish
```

Where `<VERSION LEVEL>` is one of `major`, `minor`, or `patch`

Next you need to manually make the release in github from the tag. This will kick off the build process
to build all the releases assets and store them on the release in github. 
