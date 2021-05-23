import React, { FC, useEffect } from "react";
import { TransactionWithSignature } from "../solana/transactions";
import "./MessagesView.css";

interface MessagesViewProps {
  transactions: Array<TransactionWithSignature>;
  setMidRowScrollTop: (total: number) => void;
}

const MessagesView: FC<MessagesViewProps> = ({
  transactions,
  setMidRowScrollTop,
}) => {
  const totalItemsHeight = React.useRef(0);
  const incrementItemsHeight = (height: number) => {
    console.log("item height", height);
    totalItemsHeight.current += height;
  };
  const view = transactions.map((trans) => {
    return (
      <MessagesItemView
        key={trans.signature}
        transaction={trans}
        incrementItemsHeight={incrementItemsHeight}
      />
    );
  });

  useEffect(() => {
    setMidRowScrollTop(totalItemsHeight.current);
  });

  return <>{view}</>;
};

interface MessagesItemViewProps {
  transaction: TransactionWithSignature;
  incrementItemsHeight: (count: number) => void;
}
const MessagesItemView: FC<MessagesItemViewProps> = ({
  transaction,
  incrementItemsHeight,
}) => {
  const itemHeight = React.useRef<HTMLUListElement | null>(null);
  const signature = transaction.signature?.toString();
  const meta = transaction.confirmedTransaction.meta;
  const trans = transaction.confirmedTransaction.transaction;
  let amount = 0;
  if (meta) {
    amount = meta.preBalances[0] - meta.postBalances[0];
  }
  useEffect(() => {
    incrementItemsHeight(itemHeight.current?.clientHeight ?? 0);
  });

  return (
    <ul ref={itemHeight} className="trans-item trans-meta">
      <li key={signature + "signature"}>
        <label>Tx:</label> &nbsp;
        {signature}
      </li>
      <li key={signature + "fee"}>
        <label>Fee:</label>&nbsp;
        {meta?.fee}
      </li>
      <li key={signature + "amount"}>
        <label>Sent Amount:</label>&nbsp;
        {amount}
      </li>
      <li key={signature + "sender"}>
        <label>Sender:</label>&nbsp;
        {trans.instructions[0].keys[0].pubkey.toBase58()}
      </li>
      <li key={signature + "sender-balance"}>
        <label>Sender Balance:</label>&nbsp;
        {meta?.postBalances[0]}
      </li>
      <li key={signature + "receiver"}>
        <label>Receiver:</label>&nbsp;
        {trans.instructions[0].keys[1].pubkey.toBase58()}
      </li>
      <li key={signature + "receiver-balance"}>
        <label>Receiver Balance:</label>&nbsp;
        {meta?.postBalances[1]}
      </li>
    </ul>
  );
};

export default MessagesView;