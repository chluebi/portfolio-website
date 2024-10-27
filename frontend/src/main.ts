import { setupSearch } from './searchbar.js'
import { setupProject } from './project.js'
import { Project } from './types.js'


const inputElement = document.querySelector<HTMLInputElement>('#searchBox');

const resetSearch = setupSearch(inputElement!, search);

async function search(s: String) {
  const projectsDiv = document.querySelector<HTMLDivElement>("#results");
  if (projectsDiv) {
    projectsDiv.innerHTML = "";
  }

  if (s.length > 0) {
    const response = await fetch('/api?q=' + s);
    const data = await response.json();
    const projects: Array<Project> = data.results;

    console.log(projects);

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

const url = new URL(window.location.href)
if (url.searchParams.get('s')) {
  search(new String(url.searchParams.get('s')))
}


