import * as React from "react";
import "./App.css";

const logo = require("./logo.svg");

class App extends React.Component<{}, { api_message: string }> {
  constructor(props: object) {
    super(props);

    this.state = { api_message: "" };
  }

  public componentDidMount() {
    fetch("/api").then(r => r.text()).then(api_message => {
      this.setState({
        api_message
      });
    });
  }

  public render() {
    return (
      <div className="App">
        <div className="App-header">
          <img src={logo} className="App-logo" alt="logo" />
          <h2>Welcome to React</h2>
        </div>
        <p className="App-intro">
          To get started, edit <code>src/App.tsx</code> and save to reload.
        </p>
        <p>
          {this.state.api_message}
        </p>
      </div>
    );
  }
}

export default App;
