import { portfolio } from './generated/portfolio';

export function setupProject(linkElement: HTMLAnchorElement, projectData: portfolio.Project) {
    let unfolded = false;

    linkElement.addEventListener('click', () => {
        unfolded = !unfolded;
        reload();
    })

    function reload() {
        linkElement.innerHTML = "";
        
        const titleElement = document.createElement('h2');
        titleElement.innerHTML = projectData.title;
        linkElement.appendChild(titleElement);

        if (unfolded) {

        } else {
            
        }
    }
    reload();
}