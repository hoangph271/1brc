const FILE_PATH = 'data/measurements.txt'

const fileHandler = await Deno.open(FILE_PATH)

const fileReader = fileHandler.readable

let bytesCount = 0

const start = Date.now()

for await (const chunk of fileReader) {
    bytesCount += chunk.byteLength
}

console.info(bytesCount)

console.info(Date.now() - start)
