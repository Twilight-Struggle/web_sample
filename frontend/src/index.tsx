import React, { useState } from "react";
import ReactDOM from "react-dom";
import "./index.css";

type BoardPorps = {
  boardsta: boolean[];
  gameid: string;
  setStatus: React.Dispatch<React.SetStateAction<string>>;
  setBoardsta: React.Dispatch<React.SetStateAction<boolean[]>>;
};
function Board(props: BoardPorps) {
  const [move, setMove] = useState(-1);
  const handleMove = (index: number) => {
    console.log(index);
    if (move !== -1) {
      const requestOption = {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ id: props.gameid, from: move, to: index }),
      };
      fetch("/mov", requestOption)
        .then((res) => res.json())
        .then((data) => {
          props.setStatus(data.res);
          props.setBoardsta(data.board.cells);
        });
      setMove(-1);
    } else {
      setMove(index);
    }
  };
  return (
    <div>
      <div className="board-row">
        {props.boardsta.map((piece, index) =>
          piece ? (
            <button
              key={index}
              onClick={(e) => handleMove(index)}
              className="square-piece"
            ></button>
          ) : (
            <button
              key={index}
              onClick={(e) => handleMove(index)}
              className="square"
            ></button>
          )
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
      fetch("/make", { method: "POST" })
        .then((res) => res.json())
        .then((data) => {
          setStatus(data.res);
          setGameid(data.id);
          setBoardsta(data.board.cells);
        });
      setMade(true);
    } else {
      const requestOption = {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ id: gameid, from: 0, to: 1 }),
      };
      fetch("/reset", requestOption)
        .then((res) => res.json())
        .then((data) => {
          setStatus(data.res);
          setBoardsta(data.board.cells);
        });
    }
  };

  return (
    <div className="game">
      <div className="game-board">
        <Board
          boardsta={boardsta}
          gameid={gameid}
          setStatus={setStatus}
          setBoardsta={setBoardsta}
        />
      </div>
      <div className="game-info">
        <button onClick={handleMake}>{made ? "リセット" : "作成"}</button>
        <label>{status}</label>
      </div>
    </div>
  );
}

ReactDOM.render(
  <React.StrictMode>
    <Game />
  </React.StrictMode>,
  document.getElementById("root")
);
