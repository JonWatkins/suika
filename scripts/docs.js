import TypeDoc from "typedoc";

async function main() {
  const app = new TypeDoc.Application();
  app.options.addReader(new TypeDoc.TSConfigReader());
  app.options.addReader(new TypeDoc.TypeDocReader());
  app.bootstrap();

  const project = app.convert();

  if (project) {
    const outputDir = "docs";
    await app.generateDocs(project, outputDir);
  }
}

main().catch(console.error);
