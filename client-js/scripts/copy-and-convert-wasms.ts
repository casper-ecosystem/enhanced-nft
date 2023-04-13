import * as path from "node:path";
import * as fs from "fs";

const LIST_OF_WASMS = [
  "contract.wasm",
  "mint_call.wasm",
  "balance_of_call.wasm",
  "owner_of_call.wasm",
  "get_approved_call.wasm",
  "transfer_call.wasm",
  "updated_receipts.wasm",
];

const PATH_FROM = path.resolve(__dirname, '../../tests/wasm');
const PATH_TO = path.resolve(__dirname, '../wasm');

const convertFileContent = (base64) => `
  /* Autogenerated file. Do not edit manually. */
  /* eslint-disable eslint-comments/disable-enable-pair */
  /* eslint-disable eslint-comments/no-unlimited-disable */
  /* eslint-disable */
  /* prettier-ignore */
  const base64 = "${base64}";
  const wasm = new Uint8Array(Buffer.from(base64, 'base64'));
  export default wasm;
`;

const runConversion = async () => {
  fs.rmSync(PATH_TO, { recursive: true, force: true });
  fs.mkdirSync(PATH_TO);

  LIST_OF_WASMS.forEach(async (fileName) => {
    const orgFileContent = fs.readFileSync(
      path.resolve(__dirname, `${PATH_FROM}/${fileName}`)
    );
    const base64 = orgFileContent.toString("base64");
    const convertedFileContent = convertFileContent(base64);
    const savedFilePath = path.resolve(__dirname, `../wasm/${fileName}.ts`);
    await fs.writeFileSync(savedFilePath, convertedFileContent);
  });
};

runConversion();
