import { Identity } from "@dfinity/agent";

declare global {
  interface Window {
    identity: Identity; // Replace 'any' with a more specific type if you know the structure of 'identity'
    editor: any;
    global: Window;
  }
}
