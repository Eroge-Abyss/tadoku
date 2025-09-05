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

// Utility functions
export function formatRelativeDate(date: Date): string {
  const now = new Date();

  // Get dates without time for comparison
  const dateDay = new Date(date.getFullYear(), date.getMonth(), date.getDate());
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
  const yesterday = new Date(today);
  yesterday.setDate(yesterday.getDate() - 1);

  // Format time portion consistently
  const timeFormat = new Intl.DateTimeFormat('en-US', {
    hour: 'numeric',
    minute: '2-digit',
    hour12: true,
  });

  const time = timeFormat.format(date);

  // Check if the date is today, yesterday, or older
  if (dateDay.getTime() === today.getTime()) {
    return `Today at ${time}`;
  } else if (dateDay.getTime() === yesterday.getTime()) {
    return `Yesterday at ${time}`;
  } else if (now.getTime() - date.getTime() < 7 * 24 * 60 * 60 * 1000) {
    // Within the last week, show day name
    const dayName = new Intl.DateTimeFormat('en-US', {
      weekday: 'long',
    }).format(date);
    return `${dayName} at ${time}`;
  } else {
    // Older than a week, show full date
    const dateFormat = new Intl.DateTimeFormat('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
    }).format(date);

    return `${dateFormat} at ${time}`;
  }
}
