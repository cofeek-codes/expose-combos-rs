let arg = process.argv[2]

return new Set(arg.split('').forEach(e => +e)).size
