import http2 from "http2";
import WebSocket from "ws";
import fs from "fs";
const مفتاح = ""; 
let mfaT = "";
let vanity;
const guilds = {};
const الوحدة = "";
const client = http2.connect("https://canary.discord.com");
const تَحَقّق = () => {
  fs.readFile("mfa.txt", "utf8", (err, data) => {
    if (!err) mfaT = data.trim();
  });
};
تَحَقّق();
setInterval(تَحَقّق, 4 * 60 * 1000);
const lamboreq = (code) => {
  const req = client.request({
    ":method": "PATCH",
    ":path": `/api/v10/guilds/${الوحدة}/vanity-url`,
    authorization: مفتاح,
    "x-discord-mfa-authorization": mfaT,
    "user-agent": "curl/8.4.0",
    "x-super-properties":
      "eyJvcyI6IldpbmRvd3MiLCJicm93c2VyIjoiRmlyZWZveCIsImRldmljZSI6IiIsInN5c3RlbV9sb2NhbGUiOiJ0ci1UUiIsImJyb3dzZXJfdXNlcl9hZ2VudCI6Ik1vemlsbGEvNS4wIChXaW5kb3dzIE5UIDEwLjA7IFdpbjY0OyB4NjQ7IHJ2OjEzMy4wKSBHZWNrby8yMDEwMDEwMSBGaXJlZm94LzEzMy4wIiwiYnJvd3Nlcl92ZXJzaW9uIjoiMTMzLjAiLCJvc192ZXJzaW9uIjoiMTAiLCJyZWZlcnJlciI6Imh0dHBzOi8vd3d3Lmdvb2dsZS5jb20vIiwicmVmZXJyaW5nX2RvbWFpbiI6Ind3dy5nb29nbGUuY29tIiwic2VhcmNoX2VuZ2luZSI6Imdvb2dsZSIsInJlZmVycmVyX2N1cnJlbnQiOiIiLCJyZWZlcnJpbmdfZG9tYWluX2N1cnJlbnQiOiIiLCJyZWxlYXNlX2NoYW5uZWwiOiJjYW5hcnkiLCJjbGllbnRfQnVpbGRfbnVtYmVyIjozNTYxNDAsImNsaWVudF9ldmVudF9zb3VyY2UiOm51bGwsImhhc19jbGllbnRfbW9kcyI6ZmFsc2V9==",
    "content-type": "application/json",
  });
  let responseData = "";
  req.on("data", (chunk) => (responseData += chunk));
  req.on("end", () => console.log("خاتمة :", responseData));
  req.write(JSON.stringify({ code }));
  req.end();
};
const websocketebaglan = () => {
  const ws = new WebSocket("wss://gateway.discord.gg/?v=9&encoding=json");
  ws.on("open", () => console.log("websockete baglanildi"));
  ws.on("close", () => process.exit());
  ws.on("error", () => process.exit());
  ws.on("message", (data) => {
    const message = JSON.parse(data.toString());
    const { op, t, d } = message;
    if (t === "GUILD_UPDATE") {
      const existing = guilds[d.guild_id];
      if (existing && existing !== d.vanity_url_code) {
        vanity = existing;
        lamboreq(existing);
        console.log(`Vanity changed: ${vanity}`);
      }
    }
    if (t === "READY") {
      d.guilds.forEach((g) => {
        if (g.vanity_url_code) guilds[g.id] = g.vanity_url_code;
      });
      console.log("Guilds loaded:", Object.keys(guilds).length);
    }
    if (op === 10) {
      ws.send(
        JSON.stringify({
          op: 2,
          d: {
            token: مفتاح,
            intents: 1,
            properties: { os: "linux", browser: "firefox", device: "تَحَقّق" },
          },
        })
      );
      setTimeout(() => {
        setInterval(() => {
          ws.send(JSON.stringify({ op: 1, d: null }));
        }, d.heartbeat_interval);
      }, 0);
    }
    if (op === 7) process.exit();
  });
};
setInterval(() => {
  const req = client.request({
    ":method": "HEAD",
    ":path": "/api/users/@me",
    authorization: مفتاح,
  });
  req.on("error", () => {});
  req.end();
}, 124);
websocketebaglan();
