import React, { useState } from 'react';
import ReactDOM from 'react-dom';
import './index.css';

function Board() {
  return (
    <div>
      <div className='board-row'>
        <button className='square'></button>
        <button className='square'></button>
        <button className='square'></button>
      </div>
    </div>
  );
}

function Game() {
  const [made, setMade] = useState(false);
  function handleMake() {
    if (!made) {
      fetch('/make', {method: 'POST'})
      .then(res => res.json())
      .then(data => {
        console.log(data)
      })
      setMade(true);
    }
  }

  return (
    <div className='game'>
      <div className='game-board'>
        <Board />
      </div>
      <div className='game-info'>
        <button onClick={handleMake}>{made ? "リセット": "作成"}</button>
        <label >Error</label>
      </div>
    </div>
  )
}

ReactDOM.render(
  <React.StrictMode>
    <Game />
  </React.StrictMode>,
  document.getElementById('root')
);
