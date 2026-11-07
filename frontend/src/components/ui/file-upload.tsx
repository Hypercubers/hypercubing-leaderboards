"use client"

import {
  formatBytes,
  useFileUpload,
  type FileMetadata,
  type FileWithPreview,
} from "@/hooks/use-file-upload"
import {
  Alert,
  AlertDescription,
  AlertTitle,
} from "@/components/reui/alert"

import { cn } from "@/lib/utils"
import { Button } from "@/components/ui/button"
import { PlusIcon, FileIcon, XIcon, CircleAlertIcon, FileTextIcon } from "lucide-react"
import { Attachment, AttachmentActions, AttachmentContent, AttachmentDescription, AttachmentMedia, AttachmentTitle } from "./attachment"

interface FileUploadCompactProps {
  maxFiles?: number
  maxSize?: number
  accept?: string
  multiple?: boolean
  className?: string
  onFilesChange?: (files: FileWithPreview[]) => void
}

export function FileUpload({
  maxFiles = 3,
  maxSize = 2 * 1024 * 1024, // 2MB
  accept = "image/*",
  multiple = true,
  className,
  onFilesChange,
}: FileUploadCompactProps) {
  const [
    { files, isDragging, errors },
    {
      removeFile,
      handleDragEnter,
      handleDragLeave,
      handleDragOver,
      handleDrop,
      openFileDialog,
      getInputProps,
    },
  ] = useFileUpload({
    maxFiles,
    maxSize,
    accept,
    multiple,
    onFilesChange,
  })

  const isImage = (file: File | FileMetadata) => {
    const type = file instanceof File ? file.type : file.type
    return type.startsWith("image/")
  }

  return (
    <div className={cn("w-full max-w-lg", className)}>
      {/* Compact Upload Area */}
      <div
        className={cn(
          "border-border rounded-lg flex items-center gap-3 border border-dashed p-4 transition-colors",
          isDragging
            ? "border-primary bg-primary/5"
            : "border-muted-foreground/25 hover:border-muted-foreground/50"
        )}
        onDragEnter={handleDragEnter}
        onDragLeave={handleDragLeave}
        onDragOver={handleDragOver}
        onDrop={handleDrop}
      >
        <input {...getInputProps()} className="sr-only" />

        {/* Upload Button */}
        <Button
          type="button"
          onClick={openFileDialog}
          size="sm"
          className={cn(isDragging && "animate-bounce")}
        >
          <PlusIcon className="h-4 w-4" />
          Add files
        </Button>

        {/* File Previews */}
        <div className="flex flex-1 items-center gap-2">
          {files.length === 0 ? (
            <p className="text-muted-foreground text-sm">
              Drop files here or click to browse (max {maxFiles} file)
            </p>
          ) : (
            files.map((fileItem) =>
              <Attachment>
                <AttachmentMedia>
                  <FileTextIcon/>
                </AttachmentMedia>
                <AttachmentContent>
                  <AttachmentTitle>{fileItem.file.name}</AttachmentTitle>
                  <AttachmentDescription>{formatBytes(fileItem.file.size)}</AttachmentDescription>
                </AttachmentContent>
                <AttachmentActions>
                  <Button
                  type="button"
                  onClick={() => removeFile(fileItem.id)}
                  variant="destructive"
                  size="icon"
                  // className="absolute -end-2 -top-2 size-5 rounded-full opacity-0 shadow-md transition-opacity group-hover/item:opacity-100"
                >
                  <XIcon className="size-3" />
                </Button>

                </AttachmentActions>
              </Attachment>
          ))}
        </div>

        {/* File Count */}
        {files.length > 0 && (
          <div className="text-muted-foreground shrink-0 text-xs">
            {files.length}/{maxFiles}
          </div>
        )}
      </div>

      {/* Error Messages */}
      {errors.length > 0 && (
        <Alert variant="destructive" className="mt-5">
          <CircleAlertIcon
          />
          <AlertTitle>File upload error(s)</AlertTitle>
          <AlertDescription>
            {errors.map((error, index) => (
              <p key={index} className="last:mb-0">
                {error}
              </p>
            ))}
          </AlertDescription>
        </Alert>
      )}
    </div>
  )
}
