const start = Date.now()

const FILE_PATH = 'data/measurements.txt'

// const fileHandler = await Deno.open(FILE_PATH)

// const fileReader = fileHandler.readable

// let bytesCount = 0


// for await (const chunk of fileReader) {
//     bytesCount += chunk.byteLength
// }

// console.info(bytesCount)

const fileStats = await Deno.stat(FILE_PATH)

console.info(fileStats.size)

console.info(Date.now() - start)
