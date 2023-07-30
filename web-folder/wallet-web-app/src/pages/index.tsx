import Wallet from './wallet';
import { trpc } from '../utils/trpc';
import { useState } from 'react';
const HomePage = () => {
  const { data: login, fetchStatus: fetch_login_status } = trpc.login.useQuery({ username: 'demo1', password: "welcome" });
  if (!login) {
    return <div>Loading...</div>;
  }
  return (
    <>
      <Wallet />
      <pre>{JSON.stringify(login.login.success, undefined, 2)}</pre>
    </>
  );
}

export default HomePage;
