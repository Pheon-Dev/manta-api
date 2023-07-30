import { getCookie, setCookie } from "cookies-next";
import { GetServerSidePropsContext } from "next";
// import { SessionProvider, useSession } from "next-auth/react";
import type { AppProps } from "next/app";
import Head from "next/head";
import { useState } from "react";

import {
  AppShell, Burger, ColorScheme,
  ColorSchemeProvider, Group, Header, MantineProvider, MediaQuery, Navbar, ScrollArea, useMantineTheme
} from "@mantine/core";
import type { SpotlightAction } from '@mantine/spotlight';
import { SpotlightProvider } from '@mantine/spotlight';
import { IconFileText, IconSearch } from "@tabler/icons";
// import { unstable_getServerSession } from "next-auth";
import NextApp, { AppContext } from 'next/app';
import { Notifications } from '@mantine/notifications';
import type { AppType } from 'next/app';
import { trpc } from '../utils/trpc';
import { IconBrandCodecov } from '@tabler/icons-react';
import {
  Footer,
  // Aside,
  Text,
  ActionIcon,
  // Title,
} from '@mantine/core';
import { ColorSchemeToggle, NavBar, Utilities } from "../components";

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

  return (
    <>
      <Head>
        <title>Manta Wallet</title>
        <meta name="viewport" content="minimum-scale=1, initial-scale=1, width=device-width" />
        <link rel="shortcut icon" href="/favicon.png" />
      </Head>

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
                  <ColorSchemeToggle />
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

App.getInitialProps = async (appContext: AppContext) => {
  const appProps = await NextApp.getInitialProps(appContext);
  return {
    ...appProps,
    colorScheme: getCookie('mantine-color-scheme', appContext.ctx) || 'dark',
  };
};

export default trpc.withTRPC(App);
