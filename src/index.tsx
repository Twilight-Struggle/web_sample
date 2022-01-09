import React, { useState } from 'react';
import ReactDOM from 'react-dom';
import './index.css';

type BoardPorps = {boardsta: boolean[]}
function Board(props: BoardPorps) {
  return (
    <div>
      <div className='board-row'>
        {props.boardsta.map(
          (piece, index) => piece ?
            <button key={index} className='square-piece'></button>:
            <button key={index} className='square'></button>
        )}
      </div>
    </div>
  );
}

function Game() {
  const [made, setMade] = useState(false);
  const [status, setStatus] = useState("");
  const [gameid, setGameid] = useState("");
  const [boardsta, setBoardsta] = useState<boolean[]>([false, false, false]);
  const handleMake = () => {
    if (!made) {
      fetch('/make', {method: 'POST'})
        .then(res => res.json())
        .then(data => {
          setStatus(data.res);
          setGameid(data.id);
          setBoardsta(data.board.cells);
      })
      setMade(true);
    }
    else {
      const requestOption = {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id: gameid, from: 0, to: 1 })
      }
      fetch('/reset', requestOption)
        .then(res => res.json())
        .then(data => {
          setStatus(data.res);
          setGameid(data.id);
          setBoardsta(data.board.cells);
        })
    }
  }

  return (
    <div className='game'>
      <div className='game-board'>
        <Board boardsta={boardsta}/>
      </div>
      <div className='game-info'>
        <button onClick={handleMake}>{made ? "リセット": "作成"}</button>
        <label >{status}</label>
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
