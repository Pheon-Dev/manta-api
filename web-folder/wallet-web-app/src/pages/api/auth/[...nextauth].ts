import type { NextAuthOptions } from "next-auth";
import axios from 'axios';
import NextAuth from "next-auth";
import CredentialsProvider from "next-auth/providers/credentials";
import { v4 as uuidv4 } from 'uuid';

const SECRET = "supersecret";

const authOptions: NextAuthOptions = {
  providers: [
    CredentialsProvider({
      id: "credentials",
      type: "credentials",
      name: "credentials",
      credentials: {},
      async authorize(credentials, req) {
        try {
          const { name, username, password, email } = credentials as {
            name: string;
            username: string;
            password: string;
            email: string;
          };
          if (!username || !password || !email) {
            throw new Error(`User Name | Password is Missing!`);
          }

          const url = "http://localhost:8080/api/login"
          let login = await axios.request({
            url,
            method: "POST",
            data: {
              username: `${username}`,
              password: `${password}`,
              email: `${email}`,
            },
          });

          const cookie = login.headers["set-cookie"]?.toString()?.split(" ")[0].split(";")[0];

          const user = {
            name: name,
            image: cookie,
            email: email,
          }

          const id_value = uuidv4().slice(0, 8);
          const method = `create_account`
          const id = id_value
          const aurl = "http://localhost:8080/api/rpc"
          const headers = {
            Cookie: cookie
          }

          try {

            let account = await axios.request({
              method: "POST",
              url: aurl,
              headers,
              data: {
                id: 1,
                method,
                params: {
                  data: {
                    username: `${username}`,
                    balance: `0`,
                    email: `${email}`,
                    aid: `${id}`,
                    cookie: `${cookie}`,
                  }
                }
              }
            });
            console.log(account.data)
          } catch (error) {
            console.log(error)

          }
          if (user) {
            return user
          }

          return {
            // account_data: await account.data,
            login_data: await login.data,
            user: user
          };

        } catch (error) {

          throw new Error(`${error}`);
        }
      },
    }),
  ],
  secret: `${SECRET}`,
  jwt: { secret: `${SECRET}` },
  session: { strategy: "jwt" },
  pages: { signIn: "/auth", error: "/auth/error" },
  callbacks: {
    async session({ session, token }) {
      if (session?.user)
        session.user.name = token?.name
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

