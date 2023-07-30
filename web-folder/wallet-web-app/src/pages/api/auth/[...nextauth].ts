import type { NextAuthOptions } from "next-auth";
import axios from 'axios';
import NextAuth from "next-auth";
import CredentialsProvider from "next-auth/providers/credentials";

const SECRET = "supersecret";

const authOptions: NextAuthOptions = {
  providers: [
    CredentialsProvider({
      id: "credentials",
      type: "credentials",
      name: "credentials",
      credentials: {},
      async authorize(credentials, req) {
        const { username, password } = credentials as {
          username: string;
          password: string;
        };
        if (!username || !password) {
          throw new Error(`User Name | Password is Missing!`);
        }

        const url = "http://localhost:8080/api/login"
        let login = await axios.request({
          url,
          method: "POST",
          data: {
            username: `${username}`,
            password: `${password}`,
          },
        });
        const user = await login.data

        if (user) {
          return user;
        }

        throw new Error(`Wrong User Name | Password!`);
      },
    }),
  ],
  secret: `${SECRET}`,
  jwt: { secret: `${SECRET}` },
  session: { strategy: "jwt" },
  // pages: { signIn: "/auth/Login", error: "/auth/error" },
  callbacks: {
    async session({ session, token }) {
      session.accessToken = token.accessToken
      return session;
    },
    async jwt({ token, account }) {
      if (account) {
        token.accessToken = account.access_token;
      }
      return token;
    },
  },
};

export default NextAuth(authOptions);

