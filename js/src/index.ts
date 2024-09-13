import sha256 from "sha256";

export default function hash(username: string) {
  return sha256(username);
}