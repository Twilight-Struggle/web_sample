/**
 * @jest-environment jsdom
 */

import React, { useState } from "react";
import { rest } from "msw";
import { setupServer } from "msw/node";
import { render, fireEvent, waitFor, screen } from "@testing-library/react";
import "@testing-library/jest-dom";
import { Board } from "./App";

const server = setupServer(
  rest.get("/mov", (req, ress, ctx) => {
    return ress(
      ctx.json({ res: "Good", board: { cells: [false, true, false] } })
    );
  })
);

beforeAll(() => server.listen());
afterEach(() => server.resetHandlers());
afterAll(() => server.close());

test("move test", async () => {
  const [status, setStatus] = useState("");
  const [boardsta, setBoardsta] = useState([false, false, false]);
  render(
    <Board
      boardsta={boardsta}
      gameid="OK"
      setStatus={setStatus}
      setBoardsta={setBoardsta}
    />
  );

  fireEvent.click(screen.getByTestId(0));
  fireEvent.click(screen.getByTestId(1));

  await waitFor(() => screen.getByRole("heading"));

  expect(screen.getByTestId(1)).toHaveClass("square-piece");
});
