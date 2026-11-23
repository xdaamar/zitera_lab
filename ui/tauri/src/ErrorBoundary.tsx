import { Component, type ReactNode, type ErrorInfo } from 'react'
import { AlertTriangle, RefreshCw } from 'lucide-react'

interface Props {
  children: ReactNode
}

interface State {
  hasError: boolean
  error?: Error
}

export class ErrorBoundary extends Component<Props, State> {
  state: State = { hasError: false }

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error }
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('UI Uncaught Error caught by ErrorBoundary:', error, errorInfo)
  }

  handleRetry = () => {
    this.setState({ hasError: false, error: undefined })
  }

  render() {
    if (this.state.hasError) {
      return (
        <div className="flex-1 h-full flex items-center justify-center p-6 bg-[#FCFBF8]">
          <div className="max-w-md w-full p-6 rounded-2xl border-2 border-[#E6E2D8] bg-white shadow-[0_4px_0_0_#CEC8BF] text-center space-y-4">
            <div className="w-12 h-12 rounded-xl bg-[#FEF2F2] border border-[#FCA5A5] mx-auto flex items-center justify-center text-[#DC2626]">
              <AlertTriangle className="w-6 h-6" />
            </div>
            <div>
              <h3 className="font-['Space_Grotesk'] font-bold text-base text-[#1E1A14]">
                Terjadi Kendala Tampilan
              </h3>
              <p className="text-xs text-[#786F62] font-['JetBrains_Mono'] mt-1">
                {this.state.error?.message || 'Gagal memuat komponen antarmuka.'}
              </p>
            </div>
            <button
              onClick={this.handleRetry}
              className="px-4 py-2 bg-[#0F766E] hover:bg-[#115E59] text-white text-xs font-bold rounded-xl flex items-center gap-2 mx-auto cursor-pointer transition-colors shadow-[0_2px_0_0_#042F2C]"
            >
              <RefreshCw className="w-4 h-4" />
              Muat Ulang Komponen
            </button>
          </div>
        </div>
      )
    }

    return this.props.children
  }
}
