//go:generate protoc -I $PWD/proto/ $PWD/proto/helloworld.proto --go_out=plugins=grpc:$PWD/server/go/helloworld

package main

import (
	"context"
	"fmt"
	"io"
	"log"
	"net"
	"strings"
	"time"

	"google.golang.org/grpc"

	pb "github.com/ryutah/hello-grpc/server/go/helloworld"
)

type greeterServer struct{}

func (g *greeterServer) SayHello(ctx context.Context, req *pb.HelloRequest) (*pb.HelloReply, error) {
	name := "John"
	if len(req.GetName()) > 0 {
		name = req.GetName()
	}
	return &pb.HelloReply{Message: fmt.Sprintf("Hello, %s!!", name)}, nil
}

func (g *greeterServer) GetMultiGreet(req *pb.MultiGreetRequest, stream pb.Greeter_GetMultiGreetServer) error {
	name := "John"
	if len(req.GetName()) > 0 {
		name = req.GetName()
	}

	for i := 1; i <= int(req.GetCount()); i++ {
		if err := stream.Send(&pb.MultiGreetReply{
			Index:   int32(i),
			Message: fmt.Sprintf("Hello, %s!!", name),
		}); err != nil {
			return err
		}
		time.Sleep(1 * time.Second)
	}

	return nil
}

func (g *greeterServer) CLIStreamSayHello(stream pb.Greeter_CLIStreamSayHelloServer) error {
	var names []string
	for {
		req, err := stream.Recv()
		if err == io.EOF {
			break
		} else if err != nil {
			return fmt.Errorf("failed to receive request: %v", err)
		}
		if len(req.GetName()) > 0 {
			names = append(names, req.GetName())
		} else {
			names = append(names, "John")
		}
	}
	resp := &pb.HelloReply{Message: fmt.Sprintf("Hello %s!!", strings.Join(names, ", "))}
	return stream.SendAndClose(resp)
}

func main() {
	lis, err := net.Listen("tcp", "[::]:8080")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	server := grpc.NewServer()
	pb.RegisterGreeterServer(server, new(greeterServer))
	log.Println("Start server on [::]:8080...")
	server.Serve(lis)
}
