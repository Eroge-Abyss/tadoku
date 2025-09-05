export const debounce = (callback: Function, wait = 750) => {
  let timeout: ReturnType<typeof setTimeout>;

  return (...args: any[]) => {
    clearTimeout(timeout);
    timeout = setTimeout(() => callback(...args), wait);
  };
};

export const formatNotes = (text: string): string => {
  // Regex to find URLs in text. It handles http(s), ftp, and www. links.
  const urlRegex =
    /(\b(https?|ftp):\/\/[-A-Z0-9+&@#\/%?=~_|!:,.;]*[-A-Z0-9+&@#\/%=~_|])|(\bwww\.[-A-Z0-9+&@#\/%?=~_|!:,.;]*[-A-Z0-9+&@#\/%=~_|])/gim;

  return text.replace(urlRegex, (url) => {
    // Check if the URL already has a protocol, if not, prepend 'http://' for proper linking
    const fullUrl =
      url.startsWith('http') || url.startsWith('ftp') ? url : `http://${url}`;
    return `<a href="${fullUrl}" target="_blank" rel="noopener noreferer">${url}</a>`;
  });
};
