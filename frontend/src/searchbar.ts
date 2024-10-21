import { searchCallback } from './types.ts'

export function setupSearch(searchBox: HTMLInputElement, callback: searchCallback) {
  const suggestions = ['Python', 'Rust', 'Java', 'C++'];

  enum Mode {
    Unfocused,
    FocusedIdle,
    UserTyping
  }

  let mode: Mode = Mode.Unfocused;

  let suggestionIndex = 0;
  let typingTimer: NodeJS.Timeout;
  let inactivityTimer: NodeJS.Timeout;
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
    const suggestion = suggestions[suggestionIndex] + " projects...";
    typeSuggestion(suggestion);
  
    suggestionIndex = (suggestionIndex + 1) % suggestions.length;
  }

  function resetInactivityTimer() {
    clearTimeout(inactivityTimer);
    inactivityTimer = setTimeout(() => {

      if (mode == Mode.UserTyping && searchBox.value != "") {
        resetInactivityTimer();
        return;
      }

      if (mode == Mode.UserTyping) {
        mode = Mode.FocusedIdle;
        searchBox.classList.remove('strong-text');
      }

      cycleSuggestion();
      resetInactivityTimer();
    }, inactivityDelay);
  }

  searchBox.addEventListener('focus', () => {
    mode = Mode.FocusedIdle;

    clearInterval(typingTimer);
    clearInterval(inactivityTimer);
    searchBox.value = '';

    resetInactivityTimer();
  });

  searchBox.addEventListener('blur', () => {
    mode = Mode.Unfocused;
    searchBox.classList.remove('strong-text');

    cycleSuggestion();
    resetInactivityTimer();
  });

  searchBox.addEventListener('keydown', () => {
    if (mode == Mode.Unfocused) {
      return;
    }
    else if (mode == Mode.FocusedIdle) {
      mode = Mode.UserTyping;

      clearInterval(typingTimer);
      clearTimeout(inactivityTimer);
      searchBox.value = '';
      searchBox.classList.add('strong-text');
      resetInactivityTimer();
    }
    else if (mode == Mode.UserTyping) {
      clearTimeout(inactivityTimer);
      resetInactivityTimer();
    }

    callback(searchBox.value);
  })


  resetInactivityTimer();
  cycleSuggestion();
}

