# Data Validation (JavaScript/TypeScript)

GraphBit provides JSON schema validation for ensuring data quality in AI workflows.

## Overview

The validation system helps you:
- Validate JSON data against schemas
- Ensure data quality before processing
- Catch errors early in the pipeline
- Enforce data contracts

## Basic Usage

### Validate JSON

```typescript
import { validateJson } from '@infinitibit_gmbh/graphbit';

const data = JSON.stringify({
  name: 'John Doe',
  age: 30,
  email: 'john@example.com'
});

const schema = JSON.stringify({
  type: 'object',
  properties: {
    name: { type: 'string' },
    age: { type: 'number' },
    email: { type: 'string', format: 'email' }
  },
  required: ['name', 'email']
});

const result = validateJson(data, schema);

if (result.valid) {
  console.log('Data is valid!');
} else {
  console.log('Validation errors:', result.errors);
}
```

## Validation Result

```typescript
interface ValidationResult {
  valid: boolean;           // Whether validation passed
  errors: string[];         // Array of error messages
}
```

## Common Schemas

### User Object Schema

```typescript
const userSchema = JSON.stringify({
  type: 'object',
  properties: {
    id: { type: 'string' },
    name: { type: 'string' },
    email: { type: 'string', format: 'email' },
    age: { type: 'number', minimum: 0, maximum: 150 }
  },
  required: ['id', 'name', 'email']
});
```

### Array Schema

```typescript
const arraySchema = JSON.stringify({
  type: 'array',
  items: {
    type: 'object',
    properties: {
      title: { type: 'string' },
      completed: { type: 'boolean' }
    },
    required: ['title', 'completed']
  },
  minItems: 1
});
```

## Validation in Workflows

```typescript
import { validateJson } from '@infinitibit_gmbh/graphbit';

function processUserData(jsonData: string) {
  const schema = JSON.stringify({
    type: 'object',
    properties: {
      name: { type: 'string', minLength: 1 },
      email: { type: 'string', format: 'email' },
      age: { type: 'number', minimum: 18 }
    },
    required: ['name', 'email']
  });

  const result = validateJson(jsonData, schema);

  if (!result.valid) {
    throw new Error(`Validation failed: ${result.errors.join(', ')}`);
  }

  // Process valid data
  const data = JSON.parse(jsonData);
  console.log(`Processing user: ${data.name}`);
}
```

## Complete Example

```typescript
import { validateJson } from '@infinitibit_gmbh/graphbit';

function validateAndProcess() {
  // Define schema
  const schema = JSON.stringify({
    type: 'object',
    properties: {
      products: {
        type: 'array',
        items: {
          type: 'object',
          properties: {
            id: { type: 'string' },
            name: { type: 'string' },
            price: { type: 'number', minimum: 0 }
          },
          required: ['id', 'name', 'price']
        }
      }
    },
    required: ['products']
  });

  // Data to validate
  const data = JSON.stringify({
    products: [
      { id: '1', name: 'Widget', price: 9.99 },
      { id: '2', name: 'Gadget', price: 19.99 }
    ]
  });

  // Validate
  const result = validateJson(data, schema);

  if (result.valid) {
    console.log('✓ Data is valid');
    const parsed = JSON.parse(data);
    console.log(`  Products: ${parsed.products.length}`);
  } else {
    console.log('✗ Validation failed:');
    result.errors.forEach(error => console.log(`  - ${error}`));
  }
}

validateAndProcess();
```
