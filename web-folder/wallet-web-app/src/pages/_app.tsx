import { getCookie, setCookie } from "cookies-next";
import type { AppProps } from "next/app";
import Head from "next/head";
import { useState } from "react";
import { notifications } from '@mantine/notifications';

import { ActionIcon, AppShell, Burger, ColorScheme, ColorSchemeProvider, Group, Header, MantineProvider, MediaQuery, Navbar, Text, Tooltip, useMantineTheme } from "@mantine/core";
import { IconBrandCodecov, IconLogout } from '@tabler/icons-react';
import { SessionProvider, signOut, useSession } from "next-auth/react";
import NextApp, { AppContext } from 'next/app';
import { ColorSchemeToggle, NavBar, Login } from "../components";
import { trpc } from '../utils/trpc';
import { create } from 'zustand'
import { Notifications } from '@mantine/notifications';

interface MantaState {
  email: string,
  username: string,
  name: string,
  id: string,
  balance: number,
  cookie: string,
  setCookie: (cookie: string) => void,
  setID: (id: string) => void,
  setEmail: (email: string) => void,
  setName: (name: string) => void,
  setUsername: (username: string) => void,
  send: (by: number) => void,
  deposit: (by: number) => void,
  receive: (by: number) => void,
  withdraw: (by: number) => void,
  clear: (by: number) => void,
}

export const useMantaStore = create<MantaState>((set) => ({
  balance: 30200,
  email: "jane@doe.com",
  username: "demo1",
  name: "Jane Doe",
  id: "105ef042",
  cookie: "",
  setCookie: (cookie) => set({ cookie }),
  setID: (id) => set({ id }),
  setEmail: (email) => set({ email }),
  setName: (name) => set({ name }),
  setUsername: (username) => set({ username }),
  withdraw: (args) => set((state) => ({ balance: state.balance - args })),
  send: (args) => set((state) => ({ balance: state.balance - args })),
  deposit: (args) => set((state) => ({ balance: state.balance + args })),
  receive: (args) => set((state) => ({ balance: state.balance + args })),
  clear: () => set((state) => ({ balance: state.balance - state.balance })),
}))

const App = (props: AppProps & { colorScheme: ColorScheme }) => {
  const theme = useMantineTheme();
  const [opened, setOpened] = useState(false);
  const { Component, pageProps } = props;
  const [colorScheme, setColorScheme] = useState<ColorScheme>(process.env.NODE_ENV === "production" && "dark" || props.colorScheme);

  const toggleColorScheme = (value?: ColorScheme) => {
    const nextColorScheme =
      value || (colorScheme === "dark" ? "light" : "dark");
    setColorScheme(nextColorScheme);
    setCookie("mantine-color-scheme", nextColorScheme, {
      maxAge: 60 * 60 * 24 * 30,
    });
  };

  const AppContent = () => {
    const { status, data } = useSession();

    if (status === "unauthenticated") {
      return <Login />;
    }

    return (
      <>
        <ColorSchemeProvider colorScheme={colorScheme} toggleColorScheme={toggleColorScheme}>
          <MantineProvider theme={{ colorScheme }} withGlobalStyles withNormalizeCSS>
            <Notifications />
            <AppShell
              padding="md"
              navbar={
                <Navbar p="md" hiddenBreakpoint="sm" hidden={!opened} width={{ sm: 250, lg: 300 }}>
                  <NavBar />
                </Navbar>
              }
              header={
                <Header height={70} p="sm">
                  <Group position="apart">
                    <Group>
                      <MediaQuery largerThan="sm" styles={{ display: 'none' }}>
                        <Burger
                          opened={opened}
                          onClick={() => setOpened((o) => !o)}
                          size="sm"
                          color={theme.colors.gray[6]}
                          mr="xl"
                        />
                      </MediaQuery>
                      <ActionIcon variant="gradient" gradient={{ from: 'indigo', to: 'cyan', deg: 45 }} size="md"><IconBrandCodecov size={24} /></ActionIcon>
                      <Text
                        variant="gradient"
                        gradient={{ from: 'indigo', to: 'cyan', deg: 45 }}
                        sx={{ fontFamily: 'Greycliff CF, sans-serif' }}
                        ta="center" p="xs"
                        fz="xl"
                        fw={700}
                      >{"  "}Manta Wallet</Text>
                    </Group>
                    <Group>
                      <ColorSchemeToggle />
                      <IconLogout
                        color="red"
                        size={24}
                        style={{ cursor: "pointer" }}
                        onClick={() => {
                          notifications.show({
                            color: "red",
                            title: 'Authentication',
                            message: 'Logging out ... Thanks for using Manta Wallet',
                          })
                          signOut();
                        }} />
                    </Group>
                  </Group>
                </Header>
              }
              styles={(theme) => ({
                main: { backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[8] : theme.colors.gray[0] },
              })}
            >
              <Component {...pageProps} />
            </AppShell>
          </MantineProvider>
        </ColorSchemeProvider>
      </>
    );
  }
  return (
    <>
      <Head>
        <title>Manta Wallet</title>
        <meta name="viewport" content="minimum-scale=1, initial-scale=1, width=device-width" />
        <link rel="shortcut icon" href="/favicon.png" />
      </Head>


      <SessionProvider session={pageProps.session}>
        <AppContent />
      </SessionProvider>
    </>
  )
}

App.getInitialProps = async (appContext: AppContext) => {
  const appProps = await NextApp.getInitialProps(appContext);
  return {
    ...appProps,
    colorScheme: getCookie('mantine-color-scheme', appContext.ctx) || 'dark',
  };
};

export default trpc.withTRPC(App);
