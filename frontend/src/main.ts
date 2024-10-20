import './style.css'
import { setupCounter } from './searchbar.ts'
import { exampleProject } from './testdata.ts'
import { setupProject } from './project.ts'


setupCounter(document.querySelector<HTMLInputElement>('#searchBox')!)
setupProject(document.querySelector<HTMLDivElement>('#project1')!, exampleProject)