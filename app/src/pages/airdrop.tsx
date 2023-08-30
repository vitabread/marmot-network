import type { NextPage } from "next";
import Head from "next/head";
import { AirDropView } from "../views";

const AirDrop: NextPage = (props) => {
  return (
    <div>
      <Head>
        <title>Marmot Coin AirDrop</title>
        <meta
          name="description"
          content="Marmot Coin AirDrop"
        />
      </Head>
      <AirDropView />
    </div>
  );
};

export default AirDrop;
