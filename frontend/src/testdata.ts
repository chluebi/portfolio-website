import { portfolio } from './generated/portfolio';


export const projects: Array<portfolio.Project> = Array.from({ length: 100 }, (_, index) => index + 1).map((i) => {
    const exampleProject: portfolio.Project = new portfolio.Project();
    exampleProject.id = i;
    exampleProject.title = "Project " + i;
    exampleProject.tags = ["a", "b", "c"]
    return exampleProject;
});


