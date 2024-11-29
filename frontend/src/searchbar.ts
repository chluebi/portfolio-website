import { isModuleExportName } from '../node_modules/typescript/lib/typescript.js';
import { SearchCallback } from './types.js'


function splitLastWord(str: string) {
  const lastSpaceIndex = str.lastIndexOf(' ');
  if (lastSpaceIndex === -1) {
    return { lastWord: str, everythingElse: '' }; 
  }

  const everythingElse = str.slice(0, lastSpaceIndex);
  const lastWord = str.slice(lastSpaceIndex + 1);
  return { lastWord, everythingElse };
}


export function setupSearch(searchBox: HTMLInputElement, searchCallback: SearchCallback, completionCallback: SearchCallback) {

  const completionDiv = document.querySelector<HTMLDivElement>("#search-completion");
  const suggestionDiv = document.querySelector<HTMLDivElement>("#search-suggestions");

  searchBox.addEventListener('keydown', (event) => {
    if (event.key == "Tab") {
      event.preventDefault();
      if (completionDiv) {
        searchBox.value = completionDiv.innerHTML;
      }
      searchBox.focus();
    }
  })

  searchBox.addEventListener('keyup', (event) => {
    searchBox.classList.add('strong-text');

    if (completionDiv) {
      const splitCompletion = splitLastWord(completionDiv.innerHTML);
      const splitText = splitLastWord(searchBox.value);

      if (splitCompletion.everythingElse != splitText.everythingElse || !splitCompletion.lastWord.startsWith(splitText.lastWord)) {
        completionDiv.innerHTML = searchBox.value;
      }
    }

    setTimeout(() => completionCallback(searchBox.value), 0);

    if (event.key == "Enter") {
      if (completionDiv) {
        completionDiv.innerHTML = searchBox.value;
      }
      setTimeout(() => searchCallback(searchBox.value), 0);
    }
  })

  function hardSetSearch(s: String) {
    searchBox.focus();
    searchBox.classList.add('strong-text');

    searchBox.value = s.toString();
  }

  return hardSetSearch;
}

