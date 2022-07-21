import { FoxgloveEnumSchema, FoxgloveMessageSchema } from "./types";

export function generateMarkdown(
  schemas: Iterable<FoxgloveMessageSchema>,
  enums: Iterable<FoxgloveEnumSchema>,
): string {
  const sortedSchemas = [...schemas].sort((a, b) => a.name.localeCompare(b.name));
  const sortedEnums = [...enums].sort((a, b) => a.name.localeCompare(b.name));

  return [
    `\
# Foxglove schemas

Generated by https://github.com/foxglove/schemas`,

    "## Contents\n" +
      sortedEnums
        .map((enumSchema) => `- [enum ${enumSchema.name}](#enum-${enumSchema.name.toLowerCase()})`)
        .join("\n") +
      sortedSchemas.map((schema) => `- [${schema.name}](#${schema.name.toLowerCase()})`).join("\n"),
    "----",
    sortedEnums.map(
      (enumSchema) => `\
## enum ${enumSchema.name}

name | value | description
---- | ----- | -----------
${enumSchema.values
  .map((value) => `${value.name} | ${value.value} | ${value.description ?? ""}`)
  .join("\n")}

`,
    ),
    ...sortedSchemas.map(
      (schema) => `\
## ${schema.name}

<table>
  <tr>
    <th>field</th>
    <th>type</th>
    <th>description</th>
  </tr>
${schema.fields
  .map((field) => {
    const arraySuffix =
      typeof field.array === "number" ? `[${field.array}]` : field.array === true ? "[]" : "";
    let type: string;
    switch (field.type.type) {
      case "enum":
        type = `[enum ${field.type.enum.name}](#enum-${field.type.enum.name.toLowerCase()})`;
        break;
      case "nested":
        type = `[${field.type.schema.name}](#${field.type.schema.name.toLowerCase()})`;
        break;
      case "primitive":
        type = field.type.name;
        break;
    }
    return `\
<tr>
<td>${field.name}</td>
<td>

${type}${arraySuffix}

</td>
<td>

${field.description}

</td>
</tr>`;
  })
  .join("\n")}
</table>`,
    ),
  ]
    .flat()
    .join("\n\n");
}
