import * as React from 'react';
import './App.css';

import logo from './logo.svg';

class App extends React.Component<{}, { apiMessage: string }> {
  constructor(props: object) {
    super(props);

    this.state = { apiMessage: "" };
  }

  public componentDidMount() {
    fetch("/api").then(r => r.text()).then(apiMessage => {
      this.setState({
        apiMessage
      });
    });
  }

  public render() {
    return (
      <div className="App">
        <header className="App-header">
          <img src={logo} className="App-logo" alt="logo" />
          <h1 className="App-title">Welcome to React</h1>
        </header>
        <p className="App-intro">
          To get started, edit <code>src/App.tsx</code> and save to reload.
        </p>
        <p>
          {this.state.apiMessage}
        </p>
      </div>
    );
  }
}

export default App;
