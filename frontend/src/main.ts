import './style.css'
import { setupSearch } from './searchbar.ts'
import { projects } from './testdata.ts'
import { setupProject } from './project.ts'


const inputElement = document.querySelector<HTMLInputElement>('#searchBox');

const resetSearch = setupSearch(inputElement!, search);

function search(s: String) {
  const projectsDiv = document.querySelector<HTMLDivElement>("#results");
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

  history.replaceState({}, 'Search Results', '/?s=' + s);

  resetSearch(s);
}

const url = new URL(window.location.href)
if (url.searchParams.get('s')) {
  search(new String(url.searchParams.get('s')))
}


