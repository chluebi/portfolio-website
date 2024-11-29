import { setupSearch } from './searchbar.js'
import { setupProject } from './project.js'
import { Project } from './types.js'


function splitLastWord(str: string) {
  const lastSpaceIndex = str.lastIndexOf(' ');
  if (lastSpaceIndex === -1) {
    return { lastWord: str, everythingElse: '' }; 
  }

  const everythingElse = str.slice(0, lastSpaceIndex);
  const lastWord = str.slice(lastSpaceIndex + 1);
  return { lastWord, everythingElse };
}

const inputElement = document.querySelector<HTMLInputElement>('#searchBox');

const resetSearch = setupSearch(inputElement!, search, completion);

async function search(s: String) {
  const projectsDiv = document.querySelector<HTMLDivElement>("#results");

  if (s.length > 0) {
    const response = await fetch('/api/projects?q=' + s);
    const data = await response.json();
    const projects: Array<Project> = data.results.data;

    console.log(projects);

    if (projectsDiv) {
      projectsDiv.innerHTML = "";
    }

    projects.map((p, i) => {
      const projectsElement = document.createElement('a')
      projectsElement.id = 'project' + p.id;
      projectsElement.classList.add('project');
      projectsElement.setAttribute("tabindex", "" + (i+1));
      projectsDiv?.appendChild(projectsElement);

      setupProject(projectsElement, p, search)
    })
  }

  history.replaceState({}, 'Search Results', '/?s=' + s);

  resetSearch(s);
}

async function completion(s: String) {
  const completionDiv = document.querySelector<HTMLDivElement>("#search-completion");
  const suggestionsDiv = document.querySelector<HTMLDivElement>("#search-suggestions");

  const splitText = splitLastWord(s.toString());

  if (splitText.lastWord.length > 0) {
    
    const response = await fetch('/api/completion?q=' + splitText.lastWord);
    const data = await response.json();

    const completion: string = data.results.data.completion;
    console.log('Completion: ' + completion);
    
    if (completionDiv && completion != '') {

      if (completion.startsWith(splitText.lastWord.toLowerCase())) {
        if (splitText.everythingElse == "") {
          completionDiv.innerHTML = splitText.lastWord + completion.slice(splitText.lastWord.length);
        } else {
          completionDiv.innerHTML = splitText.everythingElse + " " + splitText.lastWord + completion.slice(splitText.lastWord.length);    
        }
      }

    }

    const suggestion: string = data.results.data.suggestion;
    console.log('Suggestion: ' + completion);
    
    if (suggestionsDiv) {
      if (suggestion != '') {
        suggestionsDiv.style.display = 'flex';
  
        if (splitText.everythingElse == "") {
          suggestionsDiv.innerHTML = suggestion;
        } else {
          suggestionsDiv.innerHTML = splitText.everythingElse + " " + suggestion;        
        }
  
      } else {
        suggestionsDiv.style.display = 'none';
      }
    }
    

  }
}

const url = new URL(window.location.href)
if (url.searchParams.get('s')) {
  search(new String(url.searchParams.get('s')))
}


