// Define the Section interface globally
export interface Section {
  name: string;
  link: string;
  icon: string;
}
export interface Command {
  title: string;
  desc: string;
  code: string;
}
export interface Log {
  title: string;
  message: string;
  success: boolean;
}
