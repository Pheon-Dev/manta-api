import { Welcome } from '../components/Welcome/Welcome';
import { ColorSchemeToggle } from '../components/ColorSchemeToggle/ColorSchemeToggle';
import { trpc } from '../utils/trpc';
import { useState } from 'react';
const HomePage = () => {
  // const { data: login, fetchStatus: fetch_login_status } = trpc.login.useQuery({ username: 'demo1', password: "welcome" });
  const payments = trpc.payments.useQuery();

  if (!payments) {
    return <div>Loading...</div>;
  }
  // if (!login) {
  //   return <div>Loading...</div>;
  // }
  return (
    <>
      <Welcome />
      <ColorSchemeToggle />
      <pre>{JSON.stringify(payments.data, undefined, 2)}</pre>
    </>
  );
}

export default HomePage;
