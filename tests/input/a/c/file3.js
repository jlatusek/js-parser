function generateRandomString3(length) {
  let result = '';
  const characters =
      'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  const charactersLength = characters.length;
  for (let i = 0; i < length; i++) {
    result += characters.charAt(Math.floor(Math.random() * charactersLength));
  }
  return result;
}

function shuffleArray3(array) {
  for (let i = array.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [array[i], array[j]] = [array[j], array[i]];
  }
}

function generateRandomData3(size) {
  const data = [];
  for (let i = 0; i < size; i++) {
    data.push(generateRandomString(10));
  }
  return data;
}

function processData3(data) {
  const uniqueItems = new Set(data);
  const sortedItems = [...uniqueItems].sort();
  shuffleArray(sortedItems);
  return sortedItems;
}

function main3() {
  const dataSize = 200;
  const data = generateRandomData(dataSize);
  console.log('Generated Data:', data);

  const processedData = processData(data);
  console.log('Processed Data:', processedData);
}
