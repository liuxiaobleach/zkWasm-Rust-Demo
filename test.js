function toBytes(v) {
  if (v instanceof Uint8Array) {
    return v
  }
  if (typeof v === 'string') {
    if (isHexPrefixed(v)) {
      return hexToBytes(padToEven(stripHexPrefix(v)))
    }
    return utf8ToBytes(v)
  }
}

function padToEven(a) {
  return a.length % 2 ? `0${a}` : a
}

function utf8ToBytes(utf) {
  return new TextEncoder().encode(utf)
}

function isHexPrefixed(str) {
  return str.length >= 2 && str[0] === '0' && str[1] === 'x'
}

function hexToBytes(hex) {
  if (typeof hex !== 'string') {
    throw new TypeError('hexToBytes: expected string, got ' + typeof hex)
  }
  if (hex.length % 2) throw new Error('hexToBytes: received invalid unpadded hex')
  const array = new Uint8Array(hex.length / 2)
  for (let i = 0; i < array.length; i++) {
    const j = i * 2
    array[i] = parseHexByte(hex.slice(j, j + 2))
  }
  return array
}

function parseHexByte(hexByte) {
  if (hexByte.length !== 2) throw new Error('Invalid byte sequence')
  const byte = Number.parseInt(hexByte, 16)
  if (Number.isNaN(byte)) throw new Error('Invalid byte sequence')
  return byte
}

function stripHexPrefix(str) {
  if (typeof str !== 'string') {
    return str
  }
  return isHexPrefixed(str) ? str.slice(2) : str
}

const data = [[1],[147, 131, 3],[0, 1], [], [198], [145, 33]];
function test(data) {
  const res = [];
  for(let i in data) {
    if(!data[i].length) {
      const remainder = data.slice(i + 1);
      if(remainder.length) {
        res.push(test(remainder));
      }
      break;
    } else {
      res.push(data[i]);
    }
  }
  return res;
}

let arr = [
 [1],
 [147, 131, 3],
 [0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 2, 1, 0, 0, 8, 0, 32, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 8, 0, 0, 0, 8, 8, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 16, 0, 8, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 8, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 4, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0],
 [0, 0, 0, 0, 0, 34, 212, 115, 3, 15, 17, 109, 222, 233, 246, 180, 58, 199, 139, 163],
 [198, 163, 119, 191, 196, 235, 18, 0, 36, 168, 172, 8, 238, 242, 5, 190, 22, 184, 23, 2, 8, 18, 199, 50, 35, 232, 29, 27, 219, 151, 8, 236],
 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 245, 233, 213, 80, 195, 197, 3, 100, 214, 48, 237, 180, 117, 59, 228, 4, 205, 16, 145, 33],
 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 184, 105, 145, 198, 33, 139, 54, 193, 209, 157, 74, 46, 158, 176, 206, 54, 6, 235, 72],
 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 201, 26, 58, 253, 112, 57, 92, 212, 150, 198, 71, 213, 166, 204, 157, 75, 43, 127, 173],
 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100, 214, 65, 206, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
 [192, 42, 170, 57, 178, 35, 254, 141, 10, 14, 92, 79, 39, 234, 217, 8, 60, 117, 108, 194],
 [221, 242, 82, 173, 27, 226, 200, 155, 105, 194, 176, 104, 252, 55, 141, 170, 149, 43, 167, 241, 99, 196, 161, 22, 40, 245, 90, 77, 245, 35, 179, 239],
 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 136, 230, 160, 194, 221, 210, 111, 238, 182, 79, 3, 154, 44, 65, 41, 111, 203, 63, 86, 64],
 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 201, 26, 58, 253, 112, 57, 92, 212, 150, 198, 71, 213, 166, 204, 157, 75, 43, 127, 173],
 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 84, 66, 237, 153, 70, 225, 209],
 [160, 184, 105, 145, 198, 33, 139, 54, 193, 209, 157, 74, 46, 158, 176, 206, 54, 6, 235, 72],
 [221, 242, 82, 173, 27, 226, 200, 155, 105, 194, 176, 104, 252, 55, 141, 170, 149, 43, 167, 241, 99, 196, 161, 22, 40, 245, 90, 77, 245, 35, 179, 239],
 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 245, 233, 213, 80, 195, 197, 3, 100, 214, 48, 237, 180, 117, 59, 228, 4, 205, 16, 145, 33],
 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 136, 230, 160, 194, 221, 210, 111, 238, 182, 79, 3, 154, 44, 65, 41, 111, 203, 63, 86, 64],
 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 154, 202, 0],
 [136, 230, 160, 194, 221, 210, 111, 238, 182, 79, 3, 154, 44, 65, 41, 111, 203, 63, 86, 64],
 [196, 32, 121, 249, 74, 99, 80, 215, 230, 35, 95, 41, 23, 73, 36, 249, 40, 204, 42, 200, 24, 235, 100, 254, 216, 0, 78, 17, 95, 188, 202, 103],
 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 201, 26, 58, 253, 112, 57, 92, 212, 150, 198, 71, 213, 166, 204, 157, 75, 43, 127, 173],
 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 201, 26, 58, 253, 112, 57, 92, 212, 150, 198, 71, 213, 166, 204, 157, 75, 43, 127, 173],
 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 154, 202, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 248, 171, 189, 18, 102, 185, 30, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 89, 202, 157, 112, 255, 119, 47, 253, 132, 4, 101, 163, 133, 156, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 202, 88, 234, 188, 212, 72, 118, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 16, 159],
 [192, 42, 170, 57, 178, 35, 254, 141, 10, 14, 92, 79, 39, 234, 217, 8, 60, 117, 108, 194],
 [127, 207, 83, 44, 21, 240, 166, 219, 11, 214, 208, 224, 56, 190, 167, 29, 48, 216, 8, 199, 217, 140, 179, 191, 114, 104, 169, 91, 245, 8, 27, 101],
 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 201, 26, 58, 253, 112, 57, 92, 212, 150, 198, 71, 213, 166, 204, 157, 75, 43, 127, 173],
 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 84, 66, 237, 153, 70, 225, 209]]
