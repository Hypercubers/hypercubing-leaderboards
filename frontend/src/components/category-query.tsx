import { Button } from "./ui/button"
import { ButtonGroup } from "./ui/button-group"
import { Card, CardContent, CardHeader } from "./ui/card"
import { Field, FieldLabel } from "./ui/field"
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectSeparator, SelectTrigger, SelectValue } from "./ui/select"


function CategoryQuery() {
    return (
        <Card className="mb-2">
            <CardHeader>Filter by category</CardHeader>
            <CardContent>
                <div className="flex justify-around">
                    <Field>
                        <FieldLabel>Event</FieldLabel>
                        <Select>
                            <SelectTrigger className="max-w-1/2">
                                <SelectValue placeholder="Single speedsolve" />
                            </SelectTrigger>
                            <SelectContent position="popper">
                                <SelectGroup>
                                    <SelectLabel>Speed</SelectLabel>
                                    <SelectItem value="single">Single speedsolve</SelectItem>
                                    <SelectItem value="average">Average speedsolve</SelectItem>
                                    <SelectItem value="bld">Blindfolded</SelectItem>
                                    <SelectItem value="oh">One-handed</SelectItem>
                                </SelectGroup>
                                <SelectSeparator/>
                                <SelectGroup>
                                    <SelectLabel>Fewest moves</SelectLabel>
                                    <SelectItem value="fmc">Fewest moves</SelectItem>
                                    <SelectItem value="fmcca">Computer-assisted</SelectItem>
                                </SelectGroup>
                                <SelectSeparator/>
                                <SelectGroup>
                                    <SelectLabel>Aggregate</SelectLabel>
                                    <SelectItem value="distinct">Distinct Puzzles</SelectItem>
                                </SelectGroup>
                            </SelectContent>
                        </Select>
                    </Field>

                    <Field>
                        <FieldLabel>Piece filters allowed</FieldLabel>
                        <ButtonGroup>
                            <Button>Default</Button>
                            <Button>No</Button>
                            <Button>Yes</Button>
                        </ButtonGroup>
                    </Field>


                    <Field>
                        <FieldLabel>Macros allowed</FieldLabel>
                        <ButtonGroup>
                            <Button>Default</Button>
                            <Button>No</Button>
                            <Button>Yes</Button>
                        </ButtonGroup>
                    </Field>

                </div>

            </CardContent>
        </Card>
    )
}

export default CategoryQuery
