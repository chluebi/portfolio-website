export function setupCounter(searchBox: HTMLInputElement) {
  const suggestions = ['Python', 'Rust', 'Java', 'C++'];

  let suggestionIndex = 0;
  let typingTimer: number;
  let inactivityTimer: number;
  const inactivityDelay = 3000;

  function typeSuggestion(suggestion: string) {
    searchBox.value = '';
    searchBox.classList.add('typing-animation');
  
    let i = 0;
    typingTimer = setInterval(() => {
      if (i < suggestion.length) {
        searchBox.value += suggestion.charAt(i);
        i++;
      } else {
        clearInterval(typingTimer);
        searchBox.classList.remove('typing-animation');
      }
    }, 80);
  }

  function cycleSuggestion() {
    const suggestion = suggestions[suggestionIndex];
    typeSuggestion(suggestion);
  
    suggestionIndex = (suggestionIndex + 1) % suggestions.length;
  }

  function resetInactivityTimer() {
    clearTimeout(inactivityTimer);
    inactivityTimer = setTimeout(() => {
      cycleSuggestion();
      resetInactivityTimer();
    }, inactivityDelay);
  }

  searchBox.addEventListener('focus', () => {
    clearInterval(typingTimer);
    clearInterval(typingTimer);
    searchBox.value = '';
  });

  searchBox.addEventListener('keydown', () => {
    clearInterval(typingTimer);
    clearTimeout(inactivityTimer);
    searchBox.value = '';
    resetInactivityTimer();
  })

  searchBox.addEventListener('blur', () => {
    cycleSuggestion();
  });


  resetInactivityTimer();
  cycleSuggestion();
}

