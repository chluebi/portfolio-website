import { isModuleExportName } from '../node_modules/typescript/lib/typescript.js';
import { SearchCallback } from './types.js'

export function setupSearch(searchBox: HTMLInputElement, callback: SearchCallback) {
  const suggestions = ['Python', 'Rust', 'Java', 'C++'];

  enum Mode {
    Start,
    Suggestions,
    Focused,
    Unfocused,
    Typing
  }

  let mode: Mode = Mode.Start;

  let suggestionIndex = 0;
  let typingTimer: number;
  let inactivityTimer: number;
  const inactivityDelay = 3000;

  function typeSuggestion(suggestion: string) {
    searchBox.value = '';
    searchBox.classList.add('typing-animation');
  
    let i = 0;
    typingTimer = window.setInterval(() => {
      if (mode != Mode.Suggestions) {
        clearInterval(typingTimer);
        searchBox.classList.remove('typing-animation');
        searchBox.value = '';
        return;
      }

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
    searchBox.value = "";
    searchBox.classList.remove('strong-text');
    const suggestion = suggestions[suggestionIndex] + " projects...";
    typeSuggestion(suggestion);
  
    suggestionIndex = (suggestionIndex + 1) % suggestions.length;
  }

  function resetInactivityTimer() {
    clearTimeout(inactivityTimer);
    inactivityTimer = window.setTimeout(() => {
      if (mode == Mode.Start || mode == Mode.Focused || mode == Mode.Suggestions) {
        mode = Mode.Suggestions;
        cycleSuggestion();
        resetInactivityTimer();
        return;
      }
      resetInactivityTimer();
    }, inactivityDelay);
  }

  searchBox.addEventListener('focus', () => {
    if (mode == Mode.Focused || mode == Mode.Typing || mode == Mode.Unfocused) {
      return;
    }
    mode = Mode.Focused;
    searchBox.value = "";
    searchBox.classList.add('strong-text');
  });

  searchBox.addEventListener('blur', () => {
    mode = Mode.Unfocused;
    resetInactivityTimer();
  });

  searchBox.addEventListener('keydown', (event) => {
    if (mode == Mode.Suggestions) {
      searchBox.value = "";
    }
    mode = Mode.Typing;
    searchBox.classList.add('strong-text');

    if (event.key == "Enter") {
      setTimeout(() => callback(searchBox.value), 0);
    }
  })

  function hardSetSearch(s: String) {
    searchBox.focus();
    mode = Mode.Typing;
    searchBox.classList.add('strong-text');

    searchBox.value = s.toString();
  }


  resetInactivityTimer();

  return hardSetSearch;
}

