import { portfolio } from './generated/portfolio';

export function setupProject(projectElement: HTMLDivElement, projectData: portfolio.Project) {
    projectElement.innerHTML = projectData.title;
}