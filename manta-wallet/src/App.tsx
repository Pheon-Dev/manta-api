import { useEffect, useState } from "react";
import { Person } from "./types";

function App() {
  const [people, setPeople] = useState<Person[]>([]);

  useEffect(() => {
    fetch("http://localhost:8080/people")
      .then((res) => res.json())
      .then((people: Person[]) => setPeople(people));
  }, []);

  return (
    <div>
      {people.map((person) => (
        <p>
          {person.name} is {person.age} years old
        </p>
      ))}
    </div>
  );
}

export default App;
