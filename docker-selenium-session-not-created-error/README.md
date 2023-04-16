for <https://github.com/SeleniumHQ/docker-selenium/issues/1811>

## Not works

```bash
docker run --rm -p 4444:4444 -e SE_START_XVFB=false selenium/standalone-firefox:110.0
```

## Works
```bash
docker run --rm -p 4444:4444 selenium/standalone-firefox:110.0
```

```bash
docker run --rm -p 4444:4444 -e SE_START_XVFB=false selenium/standalone-firefox:108.0
```
