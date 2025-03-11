import { uploadFile } from "../src/storage";

test("Upload file to decentralized storage", async () => {
  const file = new File(["Hello, Internet 1.5"], "test.txt");
  const hash = await uploadFile(file);
  expect(hash).toBeTruthy();
});
